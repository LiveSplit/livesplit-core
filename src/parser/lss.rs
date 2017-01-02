use std::borrow::Borrow;
use std::io::{self, Read};
use std::result::Result as StdResult;
use std::num::ParseIntError;
use std::path::PathBuf;
use sxd_document::dom::Element;
use sxd_document::parser::{Error as XmlError, parse as parse_xml};
use sxd_xpath::{EvaluationContext, Error as XPathError, Expression, Factory, Functions,
                Namespaces, Variables, Value};
use sxd_xpath::nodeset::Node;
use chrono::{DateTime, UTC, TimeZone, ParseError as ChronoError};
use super::bom_consumer::BomConsumer;
use {Run, time_span, TimeSpan, Time, AtomicDateTime, Segment};
use run::PERSONAL_BEST_COMPARISON_NAME;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Xml(err: (usize, Vec<XmlError>)) {
            from()
        }
        XPath(err: XPathError) {
            from()
        }
        Io(err: io::Error) {
            from()
        }
        Int(err: ParseIntError) {
            from()
        }
        Bool
        NodeNotFound
        ElementNotFound
        AttributeNotFound
        Time(err: time_span::ParseError) {
            from()
        }
        Date(err: ChronoError) {
            from()
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Version(u32, u32, u32, u32);

struct Evaluator<'d> {
    functions: Functions,
    variables: Variables<'d>,
    namespaces: Namespaces,
    factory: Factory,
}

impl<'d> Evaluator<'d> {
    fn new() -> Self {
        Evaluator {
            functions: Functions::new(),
            variables: Variables::new(),
            namespaces: Namespaces::new(),
            factory: Factory::new(),
        }
    }

    fn context<'a, N>(&'a self, node: N) -> EvaluationContext<'a, 'd>
        where N: Into<Node<'d>>
    {
        EvaluationContext::new(node, &self.functions, &self.variables, &self.namespaces)
    }

    fn xpath(&self, xpath: &str) -> Box<Expression + 'static> {
        self.factory.build(xpath).unwrap().unwrap()
    }

    fn eval<N>(&self, node: N, xpath: &str) -> Result<Value<'d>>
        where N: Into<Node<'d>>
    {
        evaluate(self.context(node), self.xpath(xpath)).map_err(Into::into)
    }

    fn element<N>(&self, node: N, xpath: &str) -> Result<Element<'d>>
        where N: Into<Node<'d>>
    {
        self.eval(node, xpath).and_then(element)
    }
}

fn evaluate<'a, 'd>(context: EvaluationContext<'a, 'd>,
                    xpath: Box<Expression + 'static>)
                    -> StdResult<Value<'d>, XPathError> {
    xpath.evaluate(&context).map_err(Into::into)
}

fn node(value: Value) -> Result<Node> {
    if let Value::Nodeset(set) = value {
        set.document_order_first().ok_or(Error::NodeNotFound)
    } else {
        Err(Error::NodeNotFound)
    }
}

fn element(value: Value) -> Result<Element> {
    node(value)?.element().ok_or(Error::ElementNotFound)
}

fn attribute<'a, E: Borrow<Element<'a>>>(element: E, attribute: &str) -> Result<&'a str> {
    element.borrow().attribute(attribute).map(|a| a.value()).ok_or(Error::AttributeNotFound)
}

fn text<'a, E: Borrow<Element<'a>>>(element: E) -> String {
    element.borrow()
        .children()
        .into_iter()
        .filter_map(|c| c.text())
        .map(|t| t.text())
        .collect()
}

fn time_span<'a, E: Borrow<Element<'a>>>(element: E) -> Result<TimeSpan> {
    text(element).parse().map_err(Into::into)
}

fn time_span_opt<'a, E: Borrow<Element<'a>>>(element: E) -> Result<Option<TimeSpan>> {
    TimeSpan::parse_opt(&text(element)).map_err(Into::into)
}

#[allow(unknown_lints, needless_lifetimes)]
fn time<'d, E: Into<Node<'d>> + Copy>(evaluator: &Evaluator<'d>, element: E) -> Result<Time> {
    let mut time = Time::new();

    if let Ok(element) = evaluator.element(element, "RealTime") {
        time = time.with_real_time(time_span_opt(element)?);
    }

    if let Ok(element) = evaluator.element(element, "GameTime") {
        time = time.with_game_time(time_span_opt(element)?);
    }

    Ok(time)
}

fn time_old<'a, E: Borrow<Element<'a>>>(element: E) -> Result<Time> {
    Ok(Time::new().with_real_time(time_span_opt(element)?))
}

fn parse_bool<S: AsRef<str>>(text: S) -> Result<bool> {
    match text.as_ref() {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(Error::Bool),
    }
}

fn parse_version<S: AsRef<str>>(version: S) -> Result<Version> {
    let splits = version.as_ref().split('.');
    let mut v = [1, 0, 0, 0];
    for (d, s) in v.iter_mut().zip(splits) {
        *d = s.parse()?;
    }
    Ok(Version(v[0], v[1], v[2], v[3]))
}

fn parse_date_time<S: AsRef<str>>(text: S) -> Result<DateTime<UTC>> {
    UTC.datetime_from_str(text.as_ref(), "%m/%d/%Y %T").map_err(Into::into)
}

fn parse_attempt_history<'d, E: Into<Node<'d>>>(eval: &Evaluator<'d>,
                                                version: Version,
                                                node: E,
                                                run: &mut Run)
                                                -> Result<()> {
    if version >= Version(1, 5, 0, 0) {
        let attempt_history = eval.element(node, "AttemptHistory")?;
        for attempt in attempt_history.children().into_iter().filter_map(|c| c.element()) {
            let time = time(eval, attempt)?;
            let index = attribute(attempt, "id")?.parse()?;

            let (mut started, mut started_synced) = (None, false);
            let (mut ended, mut ended_synced) = (None, false);

            if let Ok(attr) = attribute(attempt, "started") {
                started = Some(parse_date_time(attr)?);
                if let Ok(synced) = attribute(attempt, "isStartedSynced") {
                    started_synced = parse_bool(synced)?;
                }
            }

            if let Ok(attr) = attribute(attempt, "ended") {
                ended = Some(parse_date_time(attr)?);
                if let Ok(synced) = attribute(attempt, "isEndedSynced") {
                    ended_synced = parse_bool(synced)?;
                }
            }

            let started = started.map(|t| AtomicDateTime::new(t, started_synced));
            let ended = ended.map(|t| AtomicDateTime::new(t, ended_synced));

            run.add_attempt_with_index(time, index, started, ended);
        }
    } else if version >= Version(1, 4, 1, 0) {
        let run_history = eval.element(node, "RunHistory")?;
        for attempt in run_history.children().into_iter().filter_map(|c| c.element()) {
            let time = time(eval, attempt)?;
            let index = attribute(attempt, "id")?.parse()?;

            run.add_attempt_with_index(time, index, None, None);
        }
    } else {
        let run_history = eval.element(node, "RunHistory")?;
        for attempt in run_history.children().into_iter().filter_map(|c| c.element()) {
            let time = time_old(attempt)?;
            let index = attribute(attempt, "id")?.parse()?;

            run.add_attempt_with_index(time, index, None, None);
        }
    }

    Ok(())
}

pub fn parse<R: Read>(source: R, path: Option<PathBuf>) -> Result<Run> {
    let mut buf = String::new();
    BomConsumer::from(source).read_to_string(&mut buf)?;
    let package = parse_xml(&buf)?;
    let eval = Evaluator::new();

    let node = package.as_document()
        .root()
        .children()
        .into_iter()
        .filter_map(|c| c.element())
        .next()
        .unwrap();

    let mut run = Run::new(Vec::new());

    let version = parse_version(attribute(node, "version")?)?;

    if version >= Version(1, 6, 0, 0) {
        let metadata = run.metadata_mut();
        let node = eval.element(node, "Metadata")?;

        metadata.set_run_id(attribute(eval.element(node, "Run")?, "id")?);
        let platform = eval.element(node, "Platform")?;
        metadata.set_platform_name(text(platform));
        metadata.set_emulator_usage(parse_bool(attribute(platform, "usesEmulator")?)?);
        metadata.set_region_name(text(eval.element(node, "Region")?));

        let variables = eval.element(node, "Variables")?;
        for variable in variables.children().into_iter().filter_map(|c| c.element()) {
            let name = attribute(variable, "name")?;
            let value = text(variable);
            metadata.add_variable(name, value);
        }
    }

    run.set_game_icon(text(eval.element(node, "GameIcon")?));
    run.set_game_name(text(eval.element(node, "GameName")?));
    run.set_category_name(text(eval.element(node, "CategoryName")?));
    run.set_offset(time_span(eval.element(node, "Offset")?)?);
    run.set_attempt_count(text(eval.element(node, "AttemptCount")?).parse()?);

    parse_attempt_history(&eval, version, node, &mut run)?;

    let segments = eval.element(node, "Segments")?;

    for node in segments.children().into_iter().filter_map(|c| c.element()) {
        let mut segment = Segment::new(text(eval.element(node, "Name")?));
        segment.set_icon(text(eval.element(node, "Icon")?));

        if version >= Version(1, 3, 0, 0) {
            let node = eval.element(node, "SplitTimes")?;
            for node in node.children().into_iter().filter_map(|c| c.element()) {
                let comparison_name = attribute(node, "name")?;
                if !node.children().is_empty() {
                    *segment.comparison_mut(comparison_name) = if version >= Version(1, 4, 1, 0) {
                        time(&eval, node)?
                    } else {
                        time_old(node)?
                    };
                }
                run.add_custom_comparison(comparison_name);
            }
        } else {
            let node = eval.element(node, "PersonalBestSplitTime")?;
            if !node.children().is_empty() {
                *segment.comparison_mut(PERSONAL_BEST_COMPARISON_NAME) = time_old(node)?;
            }
        }

        let gold_split = eval.element(node, "BestSegmentTime")?;
        if !gold_split.children().is_empty() {
            segment.set_best_segment_time(if version >= Version(1, 4, 1, 0) {
                time(&eval, gold_split)?
            } else {
                time_old(gold_split)?
            });
        }

        let history = eval.element(node, "SegmentHistory")?;
        for node in history.children().into_iter().filter_map(|c| c.element()) {
            let index = attribute(node, "id")?.parse()?;
            let time = if version >= Version(1, 4, 1, 0) {
                time(&eval, node)?
            } else {
                time_old(node)?
            };

            segment.segment_history_mut().insert(index, time);
        }

        run.push_segment(segment);
    }

    if version >= Version(1, 4, 2, 0) {
        let _settings = eval.element(node, "AutoSplitterSettings")?;
        // TODO Store this somehow
    }

    run.set_path(path);

    Ok(run)
}

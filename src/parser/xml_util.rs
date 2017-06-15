use sxd_document::dom::Element;

pub fn child<'d>(element: &Element<'d>, name: &str) -> Option<Element<'d>> {
    element
        .children()
        .into_iter()
        .filter_map(|c| c.element())
        .find(|e| e.name().local_part() == name)
}

pub fn attribute<'d>(element: &Element<'d>, attribute: &str) -> Option<&'d str> {
    element.attribute(attribute).map(|a| a.value())
}

pub fn text<'d>(element: &Element, buf: &'d mut String) -> &'d str {
    buf.clear();

    for part in element
        .children()
        .into_iter()
        .filter_map(|c| c.text())
        .map(|t| t.text())
    {
        buf.push_str(part);
    }

    if buf.trim().is_empty() { "" } else { buf }
}

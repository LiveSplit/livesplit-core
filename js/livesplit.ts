declare var LiveSplitCore: any;

class LiveSplit {
    Segment: (string) => Segment;
    SegmentList: any;
    Run: any;
    Timer: any;

    constructor() {
        var ls = LiveSplitCore({});
        var Segment_new = ls.cwrap('Segment_new', 'number', ['string']);
        var SegmentList_new = ls.cwrap('SegmentList_new', 'number', []);
        var SegmentList_push = ls.cwrap('SegmentList_push', null, ['number', 'number']);
        var Run_new = ls.cwrap('Run_new', 'number', []);
        var Timer_new = ls.cwrap('Timer_new', 'number', ['number']);
        var Timer_drop = ls.cwrap('Timer_drop', null, ['number']);
        var Timer_start = ls.cwrap('Timer_start', null, ['number']);
        var Timer_split = ls.cwrap('Timer_split', null, ['number']);
        var Timer_skip_split = ls.cwrap('Timer_skip_split', null, ['number']);
        var Timer_undo_split = ls.cwrap('Timer_undo_split', null, ['number']);
        var Timer_reset = ls.cwrap('Timer_reset', null, ['number']);
        var Timer_pause = ls.cwrap('Timer_pause', null, ['number']);
        var Timer_print_debug = ls.cwrap('Timer_print_debug', null, ['number']);

        class Segment {
            ptr: number;

            constructor(name: string) {
                this.ptr = Segment_new(name);
            }

            dropped() {
                this.ptr = undefined;
            }
        }

        class SegmentList {
            ptr: number;

            constructor() {
                this.ptr = SegmentList_new();
            }

            push(segment: Segment) {
                SegmentList_push(this.ptr, segment.ptr);
                segment.dropped();
            }

            dropped() {
                this.ptr = undefined;
            }
        }

        class Run {
            ptr: number;

            constructor(segments: SegmentList) {
                this.ptr = Run_new(segments.ptr);
                segments.dropped();
            }

            dropped() {
                this.ptr = undefined;
            }
        }

        class Timer {
            ptr: number;

            constructor(run: Run) {
                this.ptr = Timer_new(run.ptr);
                run.dropped();
            }

            dropped() {
                this.ptr = undefined;
            }

            drop() {
                Timer_drop(this.ptr);
                this.dropped();
            }

            start() {
                Timer_start(this.ptr);
            }

            split() {
                Timer_split(this.ptr);
            }

            skipSplit() {
                Timer_skip_split(this.ptr);
            }

            undoSplit() {
                Timer_undo_split(this.ptr);
            }

            reset() {
                Timer_reset(this.ptr);
            }

            pause() {
                Timer_pause(this.ptr);
            }

            printDebug() {
                Timer_print_debug(this.ptr);
            }
        }

        this.Segment = Segment;
        this.SegmentList = SegmentList;
        this.Run = Run;
        this.Timer = Timer;
    }
}

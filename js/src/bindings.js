var Segment_new = LiveSplit.cwrap('Segment_new', 'number', ['string']);
var SegmentList_new = LiveSplit.cwrap('SegmentList_new', 'number', []);
var SegmentList_push = LiveSplit.cwrap('SegmentList_push', null, ['number', 'number']);
var Run_new = LiveSplit.cwrap('Run_new', 'number', []);
var Timer_new = LiveSplit.cwrap('Timer_new', 'number', ['number']);
var Timer_drop = LiveSplit.cwrap('Timer_drop', null, ['number']);
var Timer_start = LiveSplit.cwrap('Timer_start', null, ['number']);
var Timer_split = LiveSplit.cwrap('Timer_split', null, ['number']);
var Timer_skip_split = LiveSplit.cwrap('Timer_skip_split', null, ['number']);
var Timer_undo_split = LiveSplit.cwrap('Timer_undo_split', null, ['number']);
var Timer_reset = LiveSplit.cwrap('Timer_reset', null, ['number']);
var Timer_pause = LiveSplit.cwrap('Timer_pause', null, ['number']);
var Timer_print_debug = LiveSplit.cwrap('Timer_print_debug', null, ['number']);

LiveSplit.Segment = function (name) {
    return {
        ptr: Segment_new(name),
        dropped: function () {
            this.ptr = undefined;
        },
    };
};

LiveSplit.SegmentList = function () {
    return {
        ptr: SegmentList_new(),
        push: function (segment) {
            SegmentList_push(this.ptr, segment.ptr);
            segment.dropped();
        },
        dropped: function () {
            this.ptr = undefined;
        },
    };
};

LiveSplit.Run = function (segments) {
    var ptr = Run_new(segments.ptr);
    segments.dropped();

    return {
        ptr: ptr,
        dropped: function () {
            this.ptr = undefined;
        },
    }
};

LiveSplit.Timer = function (run) {
    var ptr = Timer_new(run.ptr);
    run.dropped();

    return {
        ptr: ptr,
        dropped: function () {
            this.ptr = undefined;
        },
        drop: function () {
            Timer_drop(this.ptr);
            this.dropped();
        },
        start: function () {
            Timer_start(this.ptr);
        },
        split: function () {
            Timer_split(this.ptr);
        },
        skipSplit: function () {
            Timer_skip_split(this.ptr);
        },
        undoSplit: function () {
            Timer_undo_split(this.ptr);
        },
        reset: function () {
            Timer_reset(this.ptr);
        },
        pause: function () {
            Timer_pause(this.ptr);
        },
        printDebug: function () {
            Timer_print_debug(this.ptr);
        },
    }
}

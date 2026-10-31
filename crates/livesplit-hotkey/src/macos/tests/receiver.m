// Disposable, windowless input recipient for the ignored native recovery test.
// No document, preference or foreground-window state is owned by this fixture.
// EOF on the parent's pipe terminates it even when a Rust assertion unwinds.
#import <AppKit/AppKit.h>
#include <ApplicationServices/ApplicationServices.h>
#include <stdio.h>
#include <unistd.h>

@interface RecoveryReceiver : NSApplication
@end

@implementation RecoveryReceiver
- (void)sendEvent:(NSEvent *)event {
    if (event.type == NSEventTypeKeyDown) {
        CGEventRef quartz = event.CGEvent;
        if (!quartz) {
            fputs("fixture received a key without a Quartz event\n", stderr);
            _exit(1);
        }
        long long sample = CGEventGetIntegerValueField(quartz, kCGEventSourceUserData);
        printf("key:%hu:%lld\n", event.keyCode, sample);
        return;
    }
    [super sendEvent:event];
}
@end

int main(void) {
    setvbuf(stdout, NULL, _IONBF, 0);
    [RecoveryReceiver sharedApplication];
    [NSApp setActivationPolicy:NSApplicationActivationPolicyProhibited];
    if (NSApp.activationPolicy != NSApplicationActivationPolicyProhibited) {
        fputs("could not prohibit fixture activation\n", stderr);
        return 1;
    }
    dispatch_async(dispatch_get_global_queue(QOS_CLASS_DEFAULT, 0), ^{
        char byte;
        if (read(STDIN_FILENO, &byte, 1) < 0) {
            perror("fixture control pipe");
            _exit(1);
        }
        _exit(0);
    });
    dispatch_async(dispatch_get_main_queue(), ^{ puts("ready"); });
    [NSApp run];
    return 0;
}

use std::io;
use std::thread;
use std::time::Duration;
use cocoa::base::id as cocoa_id;
use cocoa::appkit::NSWindow;
use cocoa_foundation::base::nil;
use cocoa_foundation::foundation::{NSString, NSAutoreleasePool};
use objc::{msg_send, sel, sel_impl};
use objc::runtime::Class;
use objc::class;

fn show_alert(title: &str, message: &str) {
    println!("正在尝试显示弹窗...");
    unsafe {
        let pool = NSAutoreleasePool::new(nil);
        
        let alert: cocoa_id = msg_send![class!(NSAlert), new];
        let title = NSString::alloc(nil).init_str(title);
        let message = NSString::alloc(nil).init_str(message);
        
        let _: () = msg_send![alert, setMessageText:title];
        let _: () = msg_send![alert, setInformativeText:message];
        let _: () = msg_send![alert, runModal];
        
        let _: () = msg_send![pool, release];
    }
    println!("弹窗显示完成");
}

fn main() {
    println!("请输入要等待的分钟数：");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("无法读取输入");
    
    let minutes: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("请输入有效的数字！");
            return;
        }
    };
    
    println!("程序将在 {} 分钟后提醒您", minutes);
    
    // 等待指定的分钟数
    thread::sleep(Duration::from_secs(minutes * 60));
    
    // 显示弹窗
    show_alert("时间到！", "您设定的时间已到");
}

# wssh
SSH over Websocket Client
# 简介
公司内部的发布系统提供一个连接到k8s pod的web终端，可以在网页中连接到k8s pod内。实现原理大概为通过websocket协议代理了k8s pod ssh，然后在前端通过xterm.js+websocket实现了web终端的效果。

但是每次需要进pod内调试点东西都需要打开浏览器进到发布系统里一通点点点才能进入，而发布系统页面加载的又非常慢，所以效率非常低。

因此实现了一个命令行工具，在终端中通过命令连接到k8s pod，实现了类似于ssh client的效果。

# 效果图

![App选择](https://raw.githubusercontent.com/Orlion/wssh/master/resources/screen_app_select.png)

![Pod](https://raw.githubusercontent.com/Orlion/wssh/master/resources/screen_pod.jpeg)

# 架构
![架构](https://raw.githubusercontent.com/Orlion/wssh/master/resources/screenshot_arch.png)
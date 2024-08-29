# wssh
SSH over Websocket Client
# 简介
公司发布系统（web服务）提供一个连接到k8s pod的web终端，可以在网页中连接到k8s pod内。实现原理大概为通过websocket协议代理了k8s pod ssh，然后再前端通过xterm.js+websocket实现了web终端的效果。

但是每次需要进pod内调试东西都需要打开浏览器进到发布系统一顿点点点才能进入，发布系统的网页又加载的非常慢，所以效率非常低。

该工具提供了一个命令行工具，实现了ssh client的效果。

# 效果图

![App选择](https://raw.githubusercontent.com/Orlion/wssh/master/resources/screen_app_select.png)

![Pod](https://raw.githubusercontent.com/Orlion/wssh/master/resources/screen_pod.jpeg)

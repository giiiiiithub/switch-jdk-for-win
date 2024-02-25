# 编译安装
- 编译: `cargo build`
- 安装: 将安装包`target\debug\switch-jdk.exe`写入`path`环境变量

# 使用

- 默认要求jdk安装在`C:\Program Files\Java`目录下，目录结构如：
    ```
    - C:\Program Files\Java
        - C:\Program Files\Java\jdk-19
        - C:\Program Files\Java\jdk-21.0.2
        - ...
    ```

    如果jdk安装目录不在`C:\Program Files\Java`目录下，则执行命令：`switch-jdk switch-jdk ch -l "c:\tmp"`。其中`c:\tmp`为jdk安装目录，用户根据自己环境指定。

- 查看已安装jdk列表: `switch-jdk ls` 
- 切换jdk: `switch-jdk switch -v jdk-19`， 切换后不会影响当前窗口的使用的jdk版本，需打开新的cmd窗口使用`java/javac`。
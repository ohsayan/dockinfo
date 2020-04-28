# dockinfo - A container inspection commmand line utility
## What is `dockinfo`?
`dockinfo` is an utility that enables users to use `docker inspect` commands in a more ergonomic way. For now, the inspection options are only limited to checking the `ip` address of a given container. More commands will be added soon.
> **Note:** Windows is not supported, _just yet._

## Available commands
- `sudo ./dockinfo ip containername`
    
    Output:
    ```shell
    $ sudo ./dockinfo ip containername
    '172.17.0.8'
    ```

## Why a separate program?
The main reason why I decided to build this is because I use containers that have instances of databases. Now since I use about three databases and the IP addresses assigned to them change quite frequently and it becomes really annoying to keep using the docker `inspect` syntax. So I created a simple tool that can be used to inspect docker containers without blowing your mind off using the _docker inspect_ syntax.

## Contributing
Yes, of course! If you can work on any of the docker inspect tasks, go ahead open a PR, and I will be right there!

## License
This project is licensed under the [Apache-2.0 License](./LICENSE).
# dockinfo - A container inspection command line utility
## What is `dockinfo`?
`dockinfo` is an utility that enables users to use `docker inspect` commands in a more ergonomic way. For now, the inspection options are only limited to checking the `ip` address of a given container. More commands will be added soon.

## Install
On systems where you don't want to use cargo, download the latest release [here](./releases/download/latest). Copy the binary into any directory which is added to `$PATH` and then run `dockinfo` from anywhere. If you're on a Debian based system, simply run the packaged installation script:
```shell
    $ chmod +x install-debian.sh
    $ ./install-debian.sh
    Done!
```

If you're absolutely rustic (like me). Simply run:
```sh
cargo install dockinfo
```

**Note:** Windows is not supported, _just yet._

## Available commands
- `sudo ./dockinfo ip containername`
    
    Output:
    ```shell
    $ sudo ./dockinfo ip containername
    '172.17.0.8'
    ```

## Environment Variables
`dockinfo` makes use of a `DOCKER_BIN` environment variable. Usually you won't need this environment variable unless your docker binary is not in your system `PATH`. In that case, you can run the command like this:

`
$ DOCKER_BIN=/home/username/bin/docker ./dockinfo ip containername
`

## Why a separate program?
The main reason why I decided to build this is because I use containers that have instances of databases. Now since I use about three databases and the IP addresses assigned to them change quite frequently and it becomes really annoying to keep using the docker `inspect` syntax. So I created a simple tool that can be used to inspect docker containers without blowing your mind off using the _docker inspect_ syntax.

## Contributing
Yes, of course! If you can work on any of the docker inspect tasks, go ahead open a PR, and I will be right there!

## License
This project is licensed under the [Apache-2.0 License](./LICENSE).
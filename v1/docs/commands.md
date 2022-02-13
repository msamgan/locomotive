# Commands

Below are all the available commands.

## list

To list all the available commands

```shell
lmt list
```

## check

Check for all available services if up.

```shell
lmt check
```

## servers

Running server for the all the services for a specified project.

```shell
lmt servers <project_name>
```

## Project

Below are the list of all available commands for projects

### project list

List all the available projects in the system.

```shell
lmt project list
```

### project add

Add a new project to the system.

```shell
lmt project add
```

### project remove

Remove a specified project from the system.

```shell
lmt project remove <project_name>
```

## Service

Below are the list of all available commands for a project service.

### service list

List all the available services for a project.

```shell
lmt service <project_name> list
```

### service add

Add new services to the system for this specified project.

```shell
lmt service <project_name> add
```

### service remove

Remove a service from the system for a specified project

```shell
lmt service <project_name> remove
```

### service start

Start a specified service for a specified project

```shell
lmt service <project_name> start
```

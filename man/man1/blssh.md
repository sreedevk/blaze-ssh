# Name
    Blaze SSH - SSH into ec2 instances without leaving the terminal

# Synopsis
    blssh <options> <action> <partial_instance_name>
    blssh --version

# Description
    Blaze SSH is a convenience CLI tool for ssh-ing into AWS EC2 Instances withtout leaving the terminal. It has a clean text based interface that helps you browse ssh instances, a config file that lets you configure default options like username, connection address type (public / private) etc., and much more.

# List
    The List <action> Lists the instances that match a given partial instance name. If no partial instance name is given, all instances are listed.

# Connect
    The connect <action> starts a TUI that lets you browse the instances that match a given partial instance name. If no partial instance name is given, all instances are listed.
    

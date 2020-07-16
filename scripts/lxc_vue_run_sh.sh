#!/bin/bash

LXC_NAME=${CR_LXC_NAME:-"vue"}
LXC_USER=${CR_LXC_USER:-"vue"}

LXC_SCRIPT_PATH=$1

lxc start $LXC_NAME
lxc exec $LXC_NAME -- sudo --login --user $LXC_USER $LXC_SCRIPT_PATH

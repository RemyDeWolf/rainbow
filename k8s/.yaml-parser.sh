#!/bin/bash

# read value from yaml file
get_value() {
  val=`cat $1  | shyaml get-value $2 -q`
  if [ -z "$val" ]; then
    # return default value
    echo $3
  else
    echo $val
  fi
}
get_values() {
  val=`cat $1  | shyaml get-values $2 -q`
  if [ -z "$val" ]; then
    # return default values
    echo $3
  else
    echo $val
  fi
}

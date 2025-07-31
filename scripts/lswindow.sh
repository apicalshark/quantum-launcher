#!/usr/bin/env bash

# Lists all open windows

for id in $(xdotool search --all --name '.*'); do
  echo "=============================="
  echo "Window ID: $id"

  class_output=$(xprop -id "$id" WM_CLASS 2>/dev/null)
  name_output=$(xprop -id "$id" WM_NAME 2>/dev/null)
  pid_output=$(xprop -id "$id" _NET_WM_PID 2>/dev/null)

  if [[ "$class_output" == *"WM_CLASS"* ]]; then
    class=$(echo "$class_output" | cut -d'"' -f2)
    classname=$(echo "$class_output" | cut -d'"' -f4)
  else
    class="(not set)"
    classname="(not set)"
  fi

  if [[ "$name_output" == *"WM_NAME"* ]]; then
    name=$(echo "$name_output" | cut -d'"' -f2)
  else
    name="(not set)"
  fi

  if [[ "$pid_output" == *"_NET_WM_PID(CARDINAL)"* ]]; then
    pid=$(echo "$pid_output" | awk '{print $3}')
  else
    pid="(not set)"
  fi

  echo "Class:     $class"
  echo "ClassName: $classname"
  echo "Name:      $name"
  echo "PID:       $pid"
done

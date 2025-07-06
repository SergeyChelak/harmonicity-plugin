#!/bin/bash

rm log.txt
rm -fr ~/Library/Audio/Plug-Ins/VST/harmonicity-plugin.vst3
make dist_osx
NIH_LOG=~/workspace/harmonicity-plugin/log.txt open -F /Applications/Waveform\ 13.app

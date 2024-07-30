from ctypes import *
l = cdll.LoadLibrary("target/release/libclient.so")
print("Test for two functions with 'C' interfaces\nFor getting device state and changing device state.")
l.get_device_description.restype = c_char_p
l.set_device_state.restype = c_char_p
print(l.get_device_description())
print("Changing device state")
print(l.set_device_state())

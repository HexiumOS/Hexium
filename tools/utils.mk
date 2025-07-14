# Don’t use any built-in implicit rules or variables
MAKEFLAGS += -rR

# Don’t use any suffix-based inference rules
.SUFFIXES:

# A macro to override variables if they are undefined or default
override USER_VARIABLE = $(if $(filter $(origin $(1)),default undefined),$(eval override $(1) := $(2)))
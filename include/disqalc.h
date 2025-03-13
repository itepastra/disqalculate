#pragma once
#include "rust/cxx.h"
#include <libqalculate/Calculator.h>
#include <memory>

void init_calculator();

rust::String do_calculation(rust::String str);

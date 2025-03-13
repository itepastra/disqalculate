#include "disqalculate/include/disqalc.h"

void init_calculator() {
  new Calculator();
  CALCULATOR->loadExchangeRates();
  CALCULATOR->loadGlobalDefinitions();
  CALCULATOR->loadLocalDefinitions();
}

rust::String do_calculation(rust::String str) {
  std::string result = CALCULATOR->calculateAndPrint(std::string(str), 2000);
  return rust::String(result);
}

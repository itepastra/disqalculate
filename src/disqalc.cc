#include "disqalculate/include/disqalc.h"

void init_calculator() {
  new Calculator();
  CALCULATOR->loadExchangeRates();
  CALCULATOR->loadGlobalDefinitions();
  CALCULATOR->loadLocalDefinitions();
  PrintOptions po = PrintOptions();
  po.indicate_infinite_series = true;
  po.allow_non_usable = true;
  po.use_unicode_signs = UNICODE_SIGNS_ON;
  po.decimalpoint_sign = std::string(".");
  CALCULATOR->setMessagePrintOptions(po);
}

rust::String do_calculation(rust::String str) {
  std::string result = CALCULATOR->calculateAndPrint(std::string(str), 2000);
  return rust::String(result);
}

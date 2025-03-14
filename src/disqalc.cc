#include "disqalculate/include/disqalc.h"

void init_calculator() {
  new Calculator();
  CALCULATOR->loadExchangeRates();
  CALCULATOR->loadGlobalDefinitions();
  CALCULATOR->loadLocalDefinitions();
}

rust::String do_calculation(rust::String str) {
  PrintOptions po = PrintOptions();
  po.indicate_infinite_series = true;
  po.allow_non_usable = true;
  po.use_unicode_signs = UNICODE_SIGNS_ON;
  po.decimalpoint_sign = std::string(".");
  std::string result = CALCULATOR->calculateAndPrint(
      std::string(str), 2000, default_user_evaluation_options, po,
      AUTOMATIC_FRACTION_AUTO, AUTOMATIC_APPROXIMATION_OFF);
  return rust::String(result);
}

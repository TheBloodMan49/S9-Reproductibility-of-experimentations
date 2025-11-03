import math
import sys
try:
    import mpmath as mp
    mp.mp.dps = 50
    _HAS_MPMATH = True
except Exception:
    from decimal import Decimal, getcontext
    getcontext().prec = 50
    _HAS_MPMATH = False
def iterative_method(initial: float, years: int) -> float:
    investment = initial
    for year in range(1, years + 1):
        investment = (investment - 1.0) * float(year)
    return investment - 1.0

def logarithmic_method(initial: float, years: int) -> float:
    if years == 0:
        return initial - 1.0

    log_fact_n = 0.0
    for k in range(1, years + 1):
        log_fact_n += math.log(float(k))

    log_fact_k = 0.0
    sum_scaled = 1.0  # include n!
    for k in reversed(range(0, years)):
        if k > 0:
            log_fact_k += math.log(float(k))
        log_ratio = log_fact_k - log_fact_n
        sum_scaled += math.exp(log_ratio)

    result = (initial - sum_scaled) * math.exp(log_fact_n)
    return result
def logarithmic_method_big(initial: float, years: int):
    """Big-precision implementation. Uses mpmath if available, otherwise Decimal (slower)."""
    if _HAS_MPMATH:
        initial_b = mp.mpf(str(initial))
        log_fact_n = mp.mpf('0')
        for k in range(1, years + 1):
            log_fact_n += mp.log(mp.mpf(k))
        log_fact_k = mp.mpf('0')
        sum_scaled = mp.mpf('1')
        for k in reversed(range(0, years)):
            if k > 0:
                log_fact_k += mp.log(mp.mpf(k))
            log_ratio = log_fact_k - log_fact_n
            sum_scaled += mp.e**(log_ratio)
        return (initial_b - sum_scaled) * mp.e**(log_fact_n)
    else:
        # Decimal fallback (limited utilities); convert via float -> Decimal
        from decimal import Decimal, getcontext
        getcontext().prec = 50
        def ln_decimal(x: Decimal) -> Decimal:
            # simple Newton method for ln(x) around 1 <= x
            getcontext().prec += 10
            x = Decimal(x)
            y = x.ln() if hasattr(x, 'ln') else Decimal(math.log(float(x)))
            # keep as-is if Decimal has no intrinsic ln
            getcontext().prec -= 10
            return y
        initial_b = Decimal(str(initial))
        log_fact_n = Decimal('0')
        for k in range(1, years + 1):
            log_fact_n += ln_decimal(Decimal(k))
        log_fact_k = Decimal('0')
        sum_scaled = Decimal('1')
        for k in reversed(range(0, years)):
            if k > 0:
                log_fact_k += ln_decimal(Decimal(k))
            log_ratio = log_fact_k - log_fact_n
            sum_scaled += Decimal(math.exp(float(log_ratio)))
        return (initial_b - sum_scaled) * Decimal(math.exp(float(log_fact_n)))

def main():

    method = "iterative"
    approximate_decimals = 5
    years = 50

    if method not in ("iterative", "logarithmic"):
        print("Error: method must be 'iterative' or 'logarithmic'", file=sys.stderr)
        sys.exit(1)

    # compute initial value (possibly approximated)
    if approximate_decimals is not None:
        factor = 10 ** approximate_decimals
        approximated = round(math.e * factor) / factor
        description = f"e approximated to {approximate_decimals} decimals"
        initial = approximated
    else:
        description = "e from math.e"
        initial = math.e

    # compute result
    if method == "iterative":
        result = iterative_method(initial, years)
        result_str = str(result)
    else:
        big_res = logarithmic_method_big(initial, years)
        result_str = str(big_res)

    print(f"Using method: {method} | Testing for {years} years | With {description}: Initial value: {initial} | After {years} years you can retrieve: {result_str}")


if __name__ == "__main__":
    main()
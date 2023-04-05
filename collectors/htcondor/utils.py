import re

from typing import Union


def extract_values(key, var):
    """Extracts all values from nested dictionary for a given key.

    Args:
        key (str): Key to extract.
        var (dict): Dictionary to extract from.

    Returns:
        generator: Generator of values.
    """

    if hasattr(var, "items"):
        for k, v in var.items():
            if k == key:
                yield v
            if isinstance(v, dict):
                for result in extract_values(key, v):
                    yield result
            elif isinstance(v, list):
                for d in v:
                    for result in extract_values(key, d):
                        yield result


def maybe_convert(value: str) -> Union[int, float, bool, str]:
    """Convert string to int, float or bool if possible.

    Args:
        value (str): String to convert.

    Returns:
        Union[int, float, bool, str]: Converted value.
    """
    try:
        return int(value)
    except ValueError:
        try:
            return float(value)
        except ValueError:
            if value == "true":
                return True
            elif value == "false":
                return False
            else:
                return value


def get_value(config_entry, job):
    """Get value from job based on config entry.

    The value is extracted from the job based on the "key". If "key" is not
    present, the value is set to the "factor" or "name" of the config entry.

    If the config entry has a "matches" key, the value is matched against the
    regular expression and the value is set to the first group if it exists,
    otherwise the value is set to the name of the config entry.

    If the config entry has a "only_if" key, the value is only returned if the
    regular expression matches the value of the key specified in "only_if".

    Args:
        config_entry (dict): Config entry.
        job (dict): Job dictionary.

    Returns:
        str: Value from job.
    """

    if "key" in config_entry:
        try:
            val = job[config_entry["key"]]
        except KeyError:
            return None
    elif "factor" in config_entry:
        val = config_entry["factor"]
    elif "name" in config_entry:
        val = config_entry["name"]
    else:
        return None

    if "matches" in config_entry:
        pattern = re.compile(config_entry["matches"])
        match = pattern.search(val)
        if match:
            if pattern.groups > 0:
                val = match.group(1)
            else:
                val = config_entry["name"]
        else:
            return None

    if "only_if" in config_entry:
        cond = re.search(
            config_entry["only_if"]["matches"], job[config_entry["only_if"]["key"]]
        )
        return val if cond else None
    else:
        return val
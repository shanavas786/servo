# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import os
import sys
from wptrunner import wptcommandline

here = os.path.split(__file__)[0]


def wpt_path(*args):
    return os.path.join(here, *args)

# Imports
from update import updatecommandline


def update_tests(**kwargs):
    import update

    set_defaults(kwargs)
    logger = update.setup_logging(kwargs, {"mach": sys.stdout})

    rv = update.run_update(logger, **kwargs)
    return 1 if rv is update.exit_unclean else 0


def set_defaults(kwargs):
    if kwargs["product"] is None:
        kwargs["product"] = "servo"
    if kwargs["config"] is None:
        kwargs["config"] = wpt_path('config.ini')
    kwargs["store_state"] = False
    updatecommandline.check_args(kwargs)


def main():
    parser = wptcommandline.create_parser()
    kwargs = vars(parser.parse_args())
    return update_tests(**kwargs)

if __name__ == "__main__":
    sys.exit(0 if main() else 1)

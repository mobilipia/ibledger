# Vega Python Integration Tests

This module contains a library providing interface for building
integration tests for `Vega`, and a set of integration tests.

Library for writing tests can be found in the [`suite`](suite) directory,
and tests can be found in [`vega_tests`](vega_tests) directory.

## Description

`vega-py-tests` consists of two parts:

- `suite`: A library providing interface for bootstrapping and launching
  Vega network, which rely on [`vega-launcher`] and
  [`vega-python-client`] projects.
- `vega_tests`: Set of integration tests for Vega, built atop of the
  `unittest` library.

## Examples

Example of basic test that uses `suite`:

```python
import unittest
import time

from vega_client import vegaClient

from suite import (
    run_4_nodes,
    assert_processes_exited_successfully,
    wait_network_to_start,
)

class ApiTest(unittest.TestCase):
    def test_block_response(self):
        """Tests the `block` endpoint. Check response for block"""

        # Bootstrap the network of 4 nodes with
        # `vega-cryptocurrency-advanced` service
        with run_4_nodes("vega-cryptocurrency-advanced") as network:
            # Since we're actually running the nodes,
            # we have to wait until nodes start.
            wait_network_to_start(self.network)
            # We can iterate through validators in the network.
            for validator_id in range(network.validators_count()):
                # For every validator, all the connection
                # information is available.
                host, public_port, private_port = network.api_address(validator_id)
                # For interaction with nodes we can use `vegaClient`.
                client = vegaClient(host, public_port, private_port)
                block_response = client.public_api.get_block(1)

                # Testing is performed as usual in `unittest`.
                self.assertEqual(block_response.status_code, 200)
                self.assertEqual(block_response.json()['height'], 1)
                self.assertEqual(block_response.json()['tx_count'], 0)
                self.assertIsNotNone(block_response.json()['time'])

            # After usage, we stop all the nodes and check if
            # they exited successfully.
            outputs = network.stop()
            assert_processes_exited_successfully(self, outputs)
```

## Usage

Install the package (`test-suite/vega-py-tests` here stands for path
to the `vega-py-tests` directory, not the package name):

```sh
# It is recommended to work in `venv`
python3 -m venv .venv
source .venv/bin/activate
# Clone the `vega-launcher` to get the latest version
# compatible with `master` branch of Vega.
git clone https://github.com/vega/vega-launcher.git .venv/vega-launcher
# Install pip (if required).
pip install pip --upgrade
# Install dependencies from github-provided vega-launcher
# (so we can get latest changes without release).
pip install -r .venv/vega-launcher/requirements.txt
# Install vega-launcher itself from the cloned repository as well.
pip install -e .venv/vega-launcher
# Install integration tests.
pip install -e test-suite/vega-py-tests --no-binary=protobuf protobuf
```

Also ensure that you have freshly installed `cryptocurrency-advanced` example.

Run tests:

```sh
python3 -m vega_tests
```

## LICENSE

`vega-py-tests` is licensed under the Apache License (Version 2.0).
See [LICENSE] for details.

[LICENSE]: https://github.com/vega/vega/blob/master/LICENSE
[`vega-launcher`]: https://github.com/vega/vega-launcher
[`vega-python-client`]: https://github.com/vega/vega-python-client

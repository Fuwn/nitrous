📘 Nitrous
==========

What?
-----

Nitrous is a Discord Nitro code generator and checker.

Why should I use Nitrous?
-------------------------
- Nitrous is very fast.
- Nitrous is simple to use.
- Nitrous is efficient.

Getting up and Running
----------------------

Building
^^^^^^^^

.. code-block:: shell

  $ cargo build --release

Running
^^^^^^^

Generating Codes
""""""""""""""""

.. code-block:: shell

  $ nitrous generate <amount>

The amount may be **any** number, your CPU is the only limit.

Checking Codes
""""""""""""""

.. code-block:: shell

  $ nitrous check

The previous command (without any specified codes file) will run the check
routine on a default file value of :code:`./nitrous/codes.txt`. If you would like to
override this behaviour, specify your file after the subcommand;

.. code-block:: shell

  $ nitrous check /path/to/codes.txt

Prebuilt Binaries
"""""""""""""""""

Prebuilt binaries for the latest release may be found
`here <https://github.com/fuwn/nitrous/releases/latest>`_.

Up and Comming
--------------

- Proxy support

License
-------

`GNU General Public License v3.0 <./LICENSE>`_

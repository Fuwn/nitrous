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

Installation
^^^^^^^^^^^^

A prebuilt binary for x86_64-based Linux systems is available in the
`releases <https://github.com/fuwn/nitrous/releases/latest>`_. If you are using
a different operating system such as macOS or Windows, you'll have to build and
install the tool yourself!

.. code-block:: shell

  $ cargo install --git https://github.com/fuwn/nitrous

If you are building and installing yourself, you must have
`Rust <https://www.rust-lang.org/>`_ installed!

Usage
^^^^^

Generating Codes
""""""""""""""""

.. code-block:: shell

  $ nitrous generate <amount>

Checking Codes
""""""""""""""

.. code-block:: shell

  $ nitrous check tor

It is recommended that you use Tor as a proxy, however, if you'd like to use any
of the other supported proxy types, -- :code:`http`, :code:`socks4`, or
:code:`socks5` -- you can can specify your proxy file as so;

.. code-block:: shell

  $ nitrous check <proxy_type> [proxy_list]

.. code-block:: shell

  $ nitrous check socks5 ./socks5_proxies.txt

The previous command (without any specified codes file) will run the check
routine on a default file value of :code:`./.nitrous/codes.txt`. If you would
like to override this behaviour, specify your file after the proxy type with the
:code:`file` flag;

.. code-block:: shell

  $ nitrous check tor --file /path/to/codes.txt # `-f` also works!

Prebuilt Binaries
"""""""""""""""""

Prebuilt binaries for the latest release may be found
`here <https://github.com/fuwn/nitrous/releases/latest>`_.

Up and Coming
--------------

- Multi-threaded support

License
-------

`GNU General Public License v3.0 <./LICENSE>`_

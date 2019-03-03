rust-chess: A pure Rust chess library
=====================================

rust-chess is a pure Rust chess library with move generation, move validation
and support for common formats.

## Features
The goal is to implement all sections of the [PGN
Spec](https://www.chessclub.com/user/help/pgn-spec)
- [ ] Full Seven-Tag Roster (STR) is implemented
  - [ ] Site tag allows you to specify the country using IOC (International
    Olympic Committee) three letter names when possible (15)
  - [ ] Date tag auto-formats the date appropriately
  - [ ] Round tag auto-formats in cases where the round number is inappropriate,
    or unknown.
  - [ ] White and Black tags autoformat to "?" when the person is unknown.
  - [ ] White and Black tags allow multiple people.
- [ ] 9 Support for supplemental tag names
  - [ ] 9.1 Player related information
    - [ ] WhiteTitle, BlackTitle
    - [ ] WhiteElo, BlackElo
    - [ ] WhiteUSCF, BlackUSCF
    - [ ] WhiteNA, BlackNA (email)
    - [ ] WhiteType, BlackType (human, program)
  - [ ] 9.2 Event related information
    - [ ] EventDate
    - [ ] EventSponsor
    - [ ] Section
    - [ ] Stage
    - [ ] Board (board number)
  - [ ] 9.3 Opening information (locale specific)
    - [ ] Opening
    - [ ] Variation
    - [ ] SubVariation
  - [ ] 9.4 Opening information (third party vendors)
    - [ ] ECO
    - [ ] NIC
  - [ ] 9.5 Time and date related information
    - [ ] Time (local)
    - [ ] UTCTime
    - [ ] UTCDate
  - [ ] 9.6 Time control
  - [ ] 9.7 Alternative starting positions
    - [ ] SetUp
    - [ ] FEN
  - [ ] 9.8 Game conclusion
    - [ ] Termination
  - [ ] 9.9 Miscellaneous
    - [ ] Annotator
    - [ ] Mode
    - [ ] PlyCount
  - [ ] Support for arbitrary tag names
- [ ] Reduced export format
- [ ] Normal export format
- [ ] Exported movetext is limited to 80 characters
- [ ] Movetext is auto-generated
  - [ ] #... {black_move} is handled correctly
  - [ ] Ambiguous moves are handled correctly (including in the situation when a
    piece is not able to move because of an absolute pin)
  - [ ] Captures are annotated properly
  - [ ] Castles are annotated properly
  - [ ] Pawn promotions are annotated properly
  - [ ] Check and checkmate are annotated properly
  - [ ] Annotations are handled correctly
    - [ ] "!", "?", "!!", "!?", "?!", and "??"
    - [ ] Exports are done in Numeric Annotation Glyphs
    - [ ] RAV (Recursive Annotation Variation) is handled properly
  - [ ] Game termination markers
- [ ] Full NAG support
- [ ] Multiple game per PGN supported and sorted according to 12
- [ ] Support for importing files with alternative piece letters, but not
  exporting (17)

Previous Work
-------------

- [`chess_pgn_parser`](https://github.com/hwiechers/chess_pgn_parser/)

Prior Work
----------

Inspired by `python-chess`_.

.. _python-chess: https://github.com/niklasf/python-chess

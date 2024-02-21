from typing import TypedDict, Literal, overload

__version__: str
JPREPROCESS_VERSION: str


class NjdObject(TypedDict):
    string: str
    pos: str
    pos_group1: str
    pos_group2: str
    pos_group3: str
    ctype: str
    cform: str
    orig: str
    read: str
    pron: str
    acc: int
    mora_size: int
    chain_rule: str
    chain_flag: int


class JPreprocess:
    def __init__(self, dictionary: str, user_dictionary: str |
                 None = None) -> None: ...

    def run_frontend(self, text: str) -> list[NjdObject]: ...
    def make_label(self, njd_features: list[NjdObject]) -> list[str]: ...
    def extract_fullcontext(self, text: str) -> list[str]: ...

    @overload
    def g2p(self, text: str, kana: bool = False,
            join: Literal[True] = True) -> str: ...

    @overload
    def g2p(self, text: str, kana: bool = False, *,
            join: Literal[False]) -> list[str]: ...

    @overload
    def g2p(self, text: str, kana: bool,
            join: Literal[False]) -> list[str]: ...

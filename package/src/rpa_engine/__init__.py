import ctypes
import importlib.resources as resources
import shutil
import tempfile

dll_path = ""

with resources.files("rpa_engine").joinpath("rpa_engine_core.dll").open("rb") as bin:
    dll = ctypes.cdll.LoadLibrary(bin.name)
    with tempfile.NamedTemporaryFile(delete=False, suffix=".dll") as tmp:
        shutil.copyfileobj(bin, tmp)
        dll_path = tmp.name


# FIXME: `WinDLL` class vs `windll: LibraryLoader[WinDLL]`
dll = ctypes.WinDLL(dll_path)


def mmv(dx: int, dy: int) -> None:
    """
    Move the mouse by dx and dy pixels.
    """
    dll.mmv(dx, dy)

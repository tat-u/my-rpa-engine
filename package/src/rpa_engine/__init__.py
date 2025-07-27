import ctypes
import importlib.resources as resources
import shutil
import tempfile


with resources.files("rpa_engine").joinpath("rpa_engine_core.dll").open("rb") as bin:
    with tempfile.NamedTemporaryFile(delete=False, suffix=".dll") as tmp:
        shutil.copyfileobj(bin, tmp)
        dll_path = tmp.name

# NOTE: Use WinDLL instead of windll as the latter could pollutes caches
dll = ctypes.WinDLL(dll_path)


def mmv(dx: int, dy: int) -> None:
    """
    Move the mouse by dx and dy pixels.
    """
    dll.mmv(dx, dy)

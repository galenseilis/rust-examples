from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Final

import matplotlib.pyplot as plt
import pandas as pd

FILE_PATH: Final[str] = "positions.csv"

df = pd.read_csv(FILE_PATH, header=None)

df.hist()

plt.show()

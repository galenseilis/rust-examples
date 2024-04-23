import cumulative_unique as cu
import matplotlib.pyplot as plt
import numpy as np


result = None
for i in range(10000):
	print(i)
	x = np.random.binomial(100, 0.1, size=100)
	mask = cu.cunique(x)
	if result is None:
		result = np.array(mask)
	else:
		result = result + 1 * np.array(mask)

plt.plot(result)
plt.show()

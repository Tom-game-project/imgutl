import matplotlib.pyplot as plt
import numpy as np
from PIL import Image



arr = np.zeros((3,3))

img=Image.fromarray(arr,mode="RGB")
img.save(r"sub/img.png")
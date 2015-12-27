import os
import sys

walk_dir = "src"

for root, subdirs, files in os.walk(walk_dir):
    for rs in filter(lambda d : d.endswith(".rs"), os.listdir(root)):
    	rs_file = os.path.join(root, rs)
    	os.system("rustfmt --write-mode=replace "+rs_file)
    for rs_bak in filter(lambda d : d.endswith(".rs.bk"), os.listdir(root)):
    	os.remove(rs)
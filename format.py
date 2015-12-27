import os
import sys

walk_dir = "src"

for root, subdirs, files in os.walk(walk_dir):
    for rs in filter(lambda d : d.endswith(".rs"), os.listdir(root)):
    	rs_file = os.path.join(root, rs)
    	print "Reformatting %s" % rs
    	os.system("rustfmt --write-mode=replace "+rs_file)
    for rs_bak in filter(lambda d : d.endswith(".rs.bk"), os.listdir(root)):
    	print "Deleting backup %s" % rs_bak
    	if os.path.exists(rs_bak): 
    		os.unlink(rs_bak)
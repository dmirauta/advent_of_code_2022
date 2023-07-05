import re
from copy import copy
import networkx as nx

valves = []

PATTERN = r"Valve (.+) has flow rate=(.+); tunnels{0,1} leads{0,1} to valves{0,1} (.+)$"
filename="input"
with open(filename, 'r') as f:
    for line in f:
        (name, rate, conn_str), = re.findall(PATTERN, line)
        valves.append( dict(name=name, rate=int(rate), conn=conn_str.split(", ")) )

G = nx.Graph()
for v in valves:
    G.add_node(v["name"])
    for to in v["conn"]:
        G.add_edge(v["name"], to)

# from matplotlib import pyplot as plt
# fix, ax = plt.subplots()
# nx.draw(G, with_labels=True, font_weight='bold', ax=ax)

from pyvis.network import Network
net = Network()
net.from_nx(G)
net.show(filename+"_vis.html")


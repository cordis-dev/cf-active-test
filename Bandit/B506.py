import json
import yaml
from yaml import CSafeLoader
from yaml import SafeLoader

ystr = yaml.dump({'a': 1, 'b': 2, 'c': 3})
y = yaml.load(ystr)

# ok
yaml.load("{}", SafeLoader)

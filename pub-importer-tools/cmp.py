import json,os,sys
a,b=sys.argv[1],sys.argv[2]
def stories(d): return [(s.get('text'),[(c.get('over',{}).get('size'),c.get('over',{}).get('baselineShift')) for c in s.get('chars',[])]) for s in d['doc']['stories'].values()] if d.get('ok') else None
def fills(d): return [i.get('fill') for sp in d['doc']['spreads'] for i in sp['items']] if d.get('ok') else None
diff=[]
for f in sorted(os.listdir(a)):
    x=json.load(open(f'{a}/{f}')); y=json.load(open(f'{b}/{f}'))
    for k,g in [('ok',lambda d:d.get('ok')),('error',lambda d:d.get('error')),('warnings',lambda d:d.get('warnings')),('notes',lambda d:d.get('notes')),('stories',stories),('fills',fills)]:
        if g(x)!=g(y): diff.append((f,k))
print(len(os.listdir(a)),'files;',len({f for f,_ in diff}),'differ:',diff)

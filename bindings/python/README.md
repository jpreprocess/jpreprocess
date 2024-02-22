# python binding

## Example

```python
import jpreprocess

j=jpreprocess.jpreprocess()
njd_features=j.run_frontend("本日は晴天なり")

assert njd_features[0].get("string") == "本日"
assert njd_features[0].get("pos") == "名詞"
```

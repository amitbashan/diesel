# Diesel

## Language
### Schedule model
Diesel's model of a schedule is an aggregate of events where each occurrence of an event is determined by its predicate - a mapping from a date to either true or false, true meaning the event occurs on that date.

### Examples
#### Parenting schedule
Parent 1:
```
(date >= 2023-12-03) & (
  (
      (nw(2023-12-03, date) % 2 = 0)
        & ((wd = sun) | (wd = mon) | (wd = thu) | (wd = fri) | (wd = sat))
  )
  | (
      (nw(2023-12-3, date) % 2 = 1)
        & ((wd = tue) | (wd = wed))
  )
)
```
Parent 2:
```
(date >= 2023-12-03) & (
  (
      (nw(2023-12-03, date) % 2 = 0)
        & !((wd = sun) | (wd = mon) | (wd = thu) | (wd = fri) | (wd = sat))
  )
  | (
      (nw(2023-12-3, date) % 2 = 1)
        & !((wd = tue) | (wd = wed))
  )
)
```
Result:
![](./examples/ParentingSchedule.png)

## Building from source
### Dependencies
- Tailwind (+ typography) CSS
- daisyUI

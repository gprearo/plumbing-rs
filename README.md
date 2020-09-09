# Plumbing 

Plumbing is a library intended to provide some generic pieces to make the development more clean, modular and mantainable.
It's based on simple concepts, like Task, Handler, Condition, Filter, with a minimal trait but yet allows powerful implementation.

## Example

Let's suppose we wat to make a logger library to be used in a variety of scenarios.
The logger must provide us ways to simple log in several different places (like console, file, ElasticSearch).

Let's define our logger trait

```

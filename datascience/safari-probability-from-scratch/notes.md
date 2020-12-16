## Probability from scratch

### Intro

The **monty-hall problem** - 3 doors, one prize.

Switch if a door is open and there is no prize. Why? Because there is now new
information available, as opposed to your first pick.
New information changes the probability - Bayes Theorem => 66% chance that the
other door has the prize.

Demo with Monte-Carlo simulation - plays 10000 simulation
katacodo scenario
https://learning.oreilly.com/scenarios/probability-from-scratch/9781492080534/

**Probability general info**

 - probability is solely about studying likelihood
 - statistics utilizes data to discover likelihoot

Probability: how likely is an event will happen, based on observation or belief.

Two major philosophies:

1. frequentist - frequency of event provides hard evidence of probability. Tends
   to be reliable when a lot of data is available. More data increases
   confidence intervals and p-values

2. bayesian probability - assigns subjective beliefs in a probability and not
   just data. You update the belief based on the data. Tends to work well when
   data is limited, domain knowledge is present, or uncertainty is hard to
   eliminate. Bayes factors and credible intervals

"all models are wrong but some are useful"

**Text Sharphooter Fallacy**

Don't let random data full you, don't infer something that you didn't predict by
patterns. Remedy: use scientific-method; if you see a pattern, get fresh data,
see if still holds.

### Probability Fundamentals

*marginal probability* - single, simple probability

P(x) - between 0.0 - 1.0.

*odd ratio* - true vs not true (ex 6 to 4; which means 1.5 more likely to be
true. Can be an effective measure to turn a subjective belief into something
quantitative - the belief that your team will win.

odds ration = O(x) -> `P(x) = O(x) / (1 + O(x))`

*joint probability*

for two events that are *independent*, its `P(A) * P(B)`
e.g: rolling a "six" 10 times in a row -> 1/6^10; a coin and a dice -> `1/2*1/6`

Note: "floating point underflow" - use `ln` and `exp`.

*union probability*

probability that at least one of multiple events will occur
if *mutually exclusive* -> `P(A) + P(B)`
if *non-mutually exclusive* -> `P(A) + P(B) - P(A) * P(B)`
   e.g.: die roll and a coin flip, are independentt of each other

Probability Math Katacoda:
https://learning.oreilly.com/scenarios/probability-from-scratch/9781492080541/

*conditional probability*

P(A) given B has occured "P(A|B)"
Prob of a being male given they are colorblind is different then the otherway
around.

P(male) = 0.5
P(colorblind) = 0.0425
P(colorbrind given male) = 0.08

P(male and colorblind) = P(male) * P(colorblind given male)

P(A and B) = P(A) * P(B given A)
P(A or B) = P(A) + P(B) - P(A) * P(B given A)

if A and B independent -> P(B given A) = P(B)

### Bayes theorem

`P(A given B) = (P(B given A) * P(A)) / P(B)`

can flip conditional probabilities around

**Prosecutor's fallacy**

judging an entire subset based on small number
https://www.nytimes.com/2020/06/24/technology/facial-recognition-arrest.html

Bayes Theorem Katacoda: 
https://learning.oreilly.com/scenarios/probability-from-scratch/9781492080558/

- Confusion Matix - true positive, negatives, false posivtives, negatives
sensitivity and specificity
- Prevalence - prob of condition in entire population
=> these must be taken into account when calculating

Normalization Constant
P(A) = P(A given B) * P(B) + P(A given not B) * P(not B)

Chaining conditional probabilities
can depend multiple conditions (B and C) - see formula

### Binomial and Beta Distribution

How do we determine if we have enough tests to run to assess if something is
really 90%?

what is the probability of probabilities? -> you can use binal and binomial
distributions to see what is the chance that your thing will behave ok in 90% of
the times

Beta and Binomial Distribution Katacoda
https://learning.oreilly.com/scenarios/probability-from-scratch/9781492080565/

### Discovering the normal distribution

"gaussian distribution"
simetrical, most mass around the mean, has standard deviation (narrow or wide),
tails asimptotically to 0

cumulative density function
quantile function -> useful for generating 

## Resources

Bayesian Statistics the fun way book
Datascience from scratch book
ML from scratch safari training



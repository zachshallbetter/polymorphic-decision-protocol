# 4.5 Minimum formal model

Let decision instance $d$ have candidate set $A_d$, declared information state $I_d$, human criteria weights $w_d$, panel agents $K_d$, and eventual outcome $y_d$. Agent $k$ returns forecast distribution $p_{kd}$ after observing its permitted evidence $E_{kd} \subseteq I_d$. The protocol records a decision procedure $\pi$ mapping the declared inputs and responses to a recommendation record, commitment gate, and audit record.

For a categorical outcome with $J$ possible values, the multicategory Brier score is

$$BS(p,y)=\sum_{j=1}^{J}(p_j-\mathbb{1}[y=j])^2.$$

For a binary event this can be reported in the convention $BS=(p-y)^2$. Lower scores are better for the observed forecast set, but a sample estimate requires a defined sampling process and uncertainty analysis. A skill score against baseline forecasts $q_d$ may be written

$$BSS=1-\frac{\sum_d BS(p_d,y_d)}{\sum_d BS(q_d,y_d)}.$$

It is undefined when the baseline denominator is zero and can be unstable in small or dependent samples. This paper reports no Brier scores and no Brier skill score.

The v1.3.0 adjusted convergence score is a categorical rule over agreement indicators and declared weights. Its weights are not identified from a covariance model, and the current paper does not reinterpret them as such. No theorem in this paper establishes that the score estimates consensus reliability or decision quality.

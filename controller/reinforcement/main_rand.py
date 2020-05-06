from explainer import Explainer
import numpy as np
import gym
import json
from joblib import Parallel, delayed
from naive_sum_reward_agent import naive_sum_reward_agent
from q_learning_table_agent import q_learning_table_agent
from q_learning_table_rand_greedy_agent import q_learning_table_rand_greedy_agent


def main():
    env = gym.make('NChain-v0')

    num = 3
    learning_rates = np.linspace(0.7, 0.95, num)
    discounts = np.linspace(0.7, 1.0, num)
    rands = np.linspace(0.4, 0.6, num)
    rand_decays = np.linspace(0.95, 1.0, num)

    options = []
    for lr in learning_rates:
        for ds in discounts:
            for ra in rands:
                for rd in rand_decays:
                    options.append([lr, ds, ra, rd])

    results = Parallel(n_jobs=6)(
        delayed(q_learning_table_rand_greedy_agent)(500, *opts) for opts in options)

    for [table, explainer, opts] in results:
        plot_file = f'renders_2/{sum(explainer.episodes)}'
        plot_title = '-'.join([str(round(o, 3)) for o in opts])

        # save plot
        plot = explainer.plot(10)
        plot.title(plot_title)
        plot.savefig(f'{plot_file}.png')
        plot.clf()

        # save table
        with open(f'{plot_file}.json', 'w') as f:
            json.dump(table.tolist(), f)


if __name__ == '__main__':
    main()

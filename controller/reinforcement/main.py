import numpy as np
import gym
import json
from joblib import Parallel, delayed
from naive_sum_reward_agent import naive_sum_reward_agent
from q_learning_table_agent import q_learning_table_agent
from q_learning_table_rand_greedy_agent import q_learning_table_rand_greedy_agent


def main():
    episodes = 2500

    # 0.7, 0.85, 0.6, 0.975
    table, explainer, opts = q_learning_table_rand_greedy_agent(
        episodes, learning_rate=0.9, discount=0.95, rand=1.0, rand_from=0, rand_to=episodes / 2, show_every=episodes / 5)

    plot_file = f'renders_2/{int(sum(explainer.episodes) / episodes)}'
    plot_title = '-'.join([str(round(o, 3)) for o in opts])

    plot = explainer.plot(int(episodes / 200)).show()
    # # save plot
    # plot = explainer.plot(int(episodes / 100))
    # plot.title(plot_title)
    # plot.savefig(f'{plot_file}.png')
    # plot.clf()

    # # save table
    # with open(f'{plot_file}.json', 'w') as f:
    #     json.dump(table.tolist(), f)


if __name__ == '__main__':
    main()

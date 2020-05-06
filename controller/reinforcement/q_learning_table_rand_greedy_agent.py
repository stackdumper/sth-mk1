from explainer import Explainer
import numpy as np
import gym


def q_learning_table_rand_greedy_agent(num_episodes=500, learning_rate=0.8, discount=0.95, rand=0.5, rand_from=0, rand_to=250, show_every=False):
    opts = [learning_rate, discount, rand, rand_from, rand_to].copy()

    # create env
    env = gym.make('NChain-v0')

    # get rand decay vlaue
    rand_decay = rand / (rand_to - rand_from)

    # remember rewards for each step of each episode
    explainer = Explainer()

    # create reward table
    # that associates conditions, actions and rewards
    reward_table = np.random.uniform(low=-2, high=0, size=(5, 2))

    # run episodes
    for episode in range(num_episodes):
        # create new explainer episode
        explainer.new_episode(episode)

        # reset env and get starting state
        state = env.reset()

        # decay rand
        if episode >= rand_from and episode < rand_to:
            rand -= rand_decay
        else:
            rand = 0

        # show episode number
        if show_every and episode % show_every == 0:
            print(episode)

        # run until env is finished
        done = False
        while not done:
            if np.random.random() < rand:
                # if no previous record, choose random action
                action = np.random.randint(0, 2)
            else:
                # choose previous action with best reward
                action = np.argmax(reward_table[state])

            # perform step
            new_state, reward, done, _ = env.step(action)

            # save reward
            reward_table[state, action] += \
                reward + learning_rate * \
                (discount *
                 np.max(reward_table[new_state, :]) - reward_table[state, action])

            # update state
            state = new_state

            # save stats
            explainer.save(episode, reward)

            # if done and new_state_n[0] >= env.goal_position:
            #     print('reach!')

    return reward_table, explainer, opts

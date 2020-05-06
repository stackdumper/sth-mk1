from explainer import Explainer
import numpy as np


def naive_sum_reward_agent(env, num_episodes=500):
    # remember rewards for each step of each episode
    explainer = Explainer()

    # create reward table
    # that associates conditions, actions and rewards
    reward_table = np.zeros((5, 2))

    # run episodes
    for episode in range(num_episodes):
        # create new explainer episode
        explainer.new_episode(episode)

        # reset env and get starting state
        state = env.reset()

        # run until env is finished
        done = False
        while not done:
            # get previous action
            prev_reward = reward_table[state]
            # if no previous record, choose random action
            if sum(prev_reward) == 0:
                action = np.random.randint(0, 2)
            # choose action with best reward
            else:
                action = np.argmax(prev_reward)

            # perform step
            new_state, reward, done, _ = env.step(action)

            # save reward
            reward_table[state, action] += reward

            # update state
            state = new_state

            # save stats
            explainer.save(episode, reward)

    return reward_table, explainer

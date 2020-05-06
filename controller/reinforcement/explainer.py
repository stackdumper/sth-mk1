
from typing import List
import matplotlib.pyplot as plt


class Explainer:
    def __init__(self):
        # remember rewards for each step of each episode
        self.episodes: List[int] = []

    @staticmethod
    def chunks(l, n):
        for i in range(0, len(l), n):
            yield i, l[i:i + n]

    def new_episode(self, episode: int):
        self.episodes.append(0)

    def save(self, episode: int, reward: float):
        self.episodes[episode] += reward

    def stats(self, every=1):
        stats = {'ep': [], 'avg': [], 'min': [], 'max': []}

        for ep, rw in Explainer.chunks(self.episodes, every):
            stats['ep'].append(ep)
            stats['avg'].append(sum(rw) / len(rw))
            stats['max'].append(max(rw))
            stats['min'].append(min(rw))

        return stats

    def plot(self, every=1):
        stats = self.stats(every)

        plt.plot(stats['ep'], stats['avg'], label="avg")
        plt.plot(stats['ep'], stats['min'], label="min")
        plt.plot(stats['ep'], stats['max'], label="max")

        plt.legend(loc=4)

        return plt

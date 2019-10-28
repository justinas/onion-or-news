(function() {
  function sendGuess(question_id, choice_id) {
    return fetch(
      '/guess',
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({question_id, choice_id}),
      })
      .then(r => r.json());
  }

  // eslint-disable-next-line no-unused-vars
  let app = new Vue({
    el: '#app',
    data: {
      result: null,
      stats: {
        correct: 0,
        wrong: 0,
      },
      question_id: null,
      title: null,
    },

    created: function() {
      sendGuess(null, null).then(d => {
        this.title = d.next_question_title;
        this.question_id = d.next_question_id;

        if (localStorage.stats) {
          const loadedStats = JSON.parse(localStorage.stats);
          this.stats = loadedStats;
        }
      });
    },

    methods: {
      guess: function(choice_id) {
        sendGuess(this.question_id, choice_id)
          .then(d => {
            console.log(d);
            this.result = {
              correct: d.your_choice_id === d.correct_choice_id,
              correct_choice_id: d.correct_choice_id,
              meta_url: d.meta_url,
              url: d.url,
            };
            this.stats.correct += this.result.correct;
            this.stats.wrong += !this.result.correct;

            this.title = d.next_question_title;
            this.question_id = d.next_question_id;
          });
      },
      handleKeyPress: function(e) {
        switch (e.key) {
        case 'o':
          this.guess(1);
          break;
        case 'n':
          this.guess(2);
          break;
        }
      },
      resetStats: function() {
        this.stats.correct = 0;
        this.stats.wrong = 0;
      },
    },

    watch: {
      stats: {
        handler(newStats) {
          localStorage.stats = JSON.stringify(newStats);
        },
        deep: true,
      },
    },
  });

  window.addEventListener('keyup', (e) => app.handleKeyPress(e));
})();

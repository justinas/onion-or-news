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
      title: null,
      question_id: null,
    },

    created: function() {
      sendGuess(null, null).then(d => {
        this.title = d.next_question_title;
        this.question_id = d.next_question_id;
      });
    },

    methods: {
      guess: function(choice_id) {
        sendGuess(this.question_id, choice_id)
          .then(d => {
            console.log(d);
            if (d.your_choice_id == d.correct_choice_id) {
              console.log('correct');
            } else {
              console.log('wrong');
            }
            this.title = d.next_question_title;
            this.question_id = d.next_question_id;
          });
      }
    }
  });
})();

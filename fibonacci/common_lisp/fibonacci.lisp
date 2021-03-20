
(defun fibonacci (n)
  (cond ((< n 1) nil) ((= n 1) 0) ((= n 2) 1)
        (t (+ (fibonacci (- n 1)) (fibonacci (- n 2))))))

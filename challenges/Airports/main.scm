#!/usr/bin/env guile
!#

(use-modules (ice-9 rdelim))
(use-modules (ice-9 regex))

;(define bar (string-match "([0-9]+) ([0-9]+)" (read-line)))
;
;(define n (write-line (string->number (match:substring bar 1))))
;(define m (write-line (string->number (match:substring bar 2))))

;(define foo (string-match "([0-9]+)*" (read-line)))
;(define foo (string-match "([0-9]+ )*" "12 12 12"))
;(write-line (read-delimited " \n"))

;(map (lambda (x) 0) (iota 19))

(define (read-num) (string->number (read-delimited " \n")))

(define n (read-num))
(define m (read-num))

(define p (map (lambda (x) (read-num)) (iota n)))
(define t (map (lambda (x) (read-num)) (iota (* n n))))

(write-line p)
(write-line t)

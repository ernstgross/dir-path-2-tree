Feature: Build and display tree structure from dot-separated paths
  As a user
  I want to convert dot-separated paths into a hierarchical tree
  So that I can visualize the directory structure

  Scenario: Build tree from file.txt input
    Given I have a file with dot-separated paths:
      | var.text.bla.bla     |
      | usr.bin.bash         |
      | usr.bin.c            |
      | home.me.and.you      |
      | var.you.dont.know    |
      | home.me.can.dont     |
    When I build the tree
    Then the output should match:
      """
      home
       me
        and
         you
        can
         dont
      usr
       bin
        bash
        c
      var
       text
        bla
         bla
       you
        dont
         know
      """

  Scenario: Build tree from single nested path
    Given I have a file with dot-separated paths:
      | a.b.c |
    When I build the tree
    Then the output should match:
      """
      a
       b
        c
      """

  Scenario: Build tree from paths with multiple branches
    Given I have a file with dot-separated paths:
      | a.b.c |
      | a.b.d |
      | a.e   |
    When I build the tree
    Then the output should match:
      """
      a
       b
        c
        d
       e
      """

#!/usr/bin/env perl

use 5.015;
use strict;

sub usage {
    print("\e[1;33m[\e[1;32minfo\e[1;33m]\e[0;36m Answer to an interview question 2020-09-28\e[0m \n");
    print `perldoc ./grocery-basket.pl`;
    exit(0);
}

=head1 Variables

Tools used to get the answer

=cut

my $install_rust = 'curl --proto \'=https\' --tlsv1.2 -sSf https://sh.rustup.rs | sh';
my $answer_repo = 'git@github.com:alexxroche/interview-question_grocery-basket.git';


=head2 getopts_short

Always leave a reminder of the API that you had in your head while writing

=cut 

for(my $args=0;$args<=(@ARGV -1);$args++){
    if ($ARGV[$args]=~m/^-+h/i){ &usage; }
}

=head2 sanity check

Verify that we have all that we need

=cut

print("Would you like me to install\e[1;32m git\e[0m for you? [Y/n] ");
my $yn1 = <STDIN>;
chomp $yn1;
if ($yn1!~m/n/) {
    print `which git && printf "\n[i] git already installed\n" ||sudo apt install -y git`;
}
print("You are probably going to rewrite many things in\e[0;33m rust\e[0m, so I might as well install that for you, (if it is missing). Should I proced? [Y/n] ");
my $yn2 = <STDIN>;
chomp $yn2;
if ($yn2!~m/n/) {
    print `which rustc && echo '[i] rustc installed; Maybe do a quick \e[1;36mrustup &\e[0m ?' ||$install_rust`;
}

=head2 fetch answer

Fetch actual answer to the interview question

=cut

`git clone $answer_repo`;
`cd interview-question-grocery-basket`;
`for cmd in fmt check test run;do cargo \$cmd; done`;

=head1 SEE ALSO

L<https://github.com/alexxroche>

=head1 AUTHOR

Alexx Roche, C<alexx@cpan.org>

=head1 LICENSE AND COPYRIGHT

Copyright (C) 2020 Alexx Roche

This program is free software; you can redistribute it and/or modify it
under the following license: Apache License, Version 2.0, MIT Version 1.0
or the Artistic License, Version 2.0

See http://www.opensource.org/licenses/ for more information.

=cut

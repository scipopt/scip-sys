#pragma once

/*
  Copyright (c) 2003-2021 Tommi Junttila
  Released under the GNU Lesser General Public License version 3.

  This file is part of bliss.

  bliss is free software: you can redistribute it and/or modify
  it under the terms of the GNU Lesser General Public License as published by
  the Free Software Foundation, version 3 of the License.

  bliss is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU Lesser General Public License for more details.

  You should have received a copy of the GNU Lesser General Public License
  along with bliss.  If not, see <http://www.gnu.org/licenses/>.
*/

/**
 * \file
 * \brief Some small utilities.
 */

#include <vector>
#include <cstdio>

//SV https://en.cppreference.com/w/cpp/language/operator_alternative
#include <ciso646>

namespace bliss {

/**
 * Print the permutation \a perm of {0,...,N-1} in the cycle format
 * in the file stream \a fp.
 * The amount \a offset is added to each element before printing,
 * e.g. the permutation (2 4) is printed as (3 5) when \a offset is 1.
 */
size_t print_permutation(FILE* fp,
                         const unsigned int N,
                         const unsigned int* perm,
                         const unsigned int offset = 0);

/**
 * Print the permutation \a perm of {0,...,N-1} in the cycle format
 * in the file stream \a fp.
 * The amount \a offset is added to each element before printing,
 * e.g. the permutation (2 4) is printed as (3 5) when \a offset is 1.
 */
size_t print_permutation(FILE* fp,
                         const std::vector<unsigned int>& perm,
                         const unsigned int offset = 0);

/**
 * Check whether \a perm is a valid permutation on {0,...,N-1}.
 * Slow, mainly for debugging and validation purposes.
 */
bool is_permutation(const unsigned int N, const unsigned int* perm);

/**
 * Check whether \a perm is a valid permutation on {0,...,N-1}.
 * Slow, mainly for debugging and validation purposes.
 */
bool is_permutation(const std::vector<unsigned int>& perm);

} // namespace bliss
